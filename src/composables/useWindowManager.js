import { reactive, markRaw, ref } from 'vue';

const state = reactive({
    windows: [],
    activeWindow: null,
});

const ready = ref(false);

const modules = import.meta.glob('@/components/windows/*.vue');

// Automatically register all windows
function initializeWindows() {
    Promise.all(
        Object.entries(modules).map(([path, loader]) =>
            loader().then((module) => {
                const id = path.match(/\/([^/]+)\.vue$/)[1];
                const title = module.default?.title || id;
                const startActive = module.default?.startActive || false;

                if (!state.windows.find((w) => w.id === id)) {
                    state.windows.push({
                        id,
                        title,
                        component: markRaw(module.default),
                    });

                    if (startActive) {
                        state.activeWindow = id;
                    }
                }
            })
        )
    ).then(() => {
        if (!state.activeWindow && state.windows.length > 0) {
            state.activeWindow = state.windows[0].id;
        }
        state.windows.sort((a, b) => a.id.localeCompare(b.id));
        ready.value = true;
    });
}

function setActiveWindow(id) {
    state.activeWindow = id;
}

export function useWindowManager() {
    return {
        state,
        initializeWindows,
        setActiveWindow,
        ready,
    };
}

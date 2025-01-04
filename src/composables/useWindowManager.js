import { markRaw, computed, ref } from 'vue';

const windowList = ref([]);
const activeWindow = ref(null);
const direction = ref('up');
const ready = ref(false);

const modules = import.meta.glob('@/components/windows/*.vue');

export function useWindowManager() {
    async function initializeWindows(startActive) {
        const moduleEntries = Object.entries(modules);
        await Promise.all(
            moduleEntries.map(async ([path, loader]) => {
                const module = await loader();
                const id = path.match(/\/([^/]+)\.vue$/)[1];
                const title = id.replace(/([A-Z])/g, ' $1').trim();

                if (!windowList.value.find((w) => w.id === id)) {
                    windowList.value.push({
                        id,
                        title,
                        component: markRaw(module.default),
                    });
                }
            })
        );
        if (windowList.value.length === 0) return;

        windowList.value.sort((a, b) => a.id.localeCompare(b.id));

        if (startActive) {
            activeWindow.value = windowList.value.find((w) => w.id === startActive)?.id;
        } else {
            activeWindow.value = windowList.value[0].id;
        }

        ready.value = true;
    }

    function moveUp() {
        direction.value = 'up';
        activeWindow.value = windowList.value[getCircularIndex(activeIndex.value - 1)].id;
    }

    function moveDown() {
        direction.value = 'down';
        activeWindow.value = windowList.value[getCircularIndex(activeIndex.value + 1)].id;
    }

    function getCircularIndex(index) {
        return (index + totalWindows.value) % totalWindows.value;
    }

    const totalWindows = computed(() => windowList.value.length);

    const activeIndex = computed(() => {
        return windowList.value.findIndex((win) => win.id === activeWindow.value)
    });

    return {
        initializeWindows,
        moveUp,
        moveDown,
        getCircularIndex,
        totalWindows,
        activeIndex,
        windowList,
        direction,
        activeWindow,
        ready,
    };
}

import { reactive, markRaw } from "vue";

const state = reactive({
    windows: [],
    activeWindow: null,
});

// Import all components from the windows directory
const modules = import.meta.glob("../components/windows/*.vue");

// Helper function to find a component by ID
function resolveComponentById(id) {
    const path = `../components/windows/${id}.vue`;
    if (modules[path]) {
        // Return a Promise that resolves the component
        return modules[path]().then((module) => markRaw(module.default));
    } else {
        console.error(`Component with id '${id}' not found in /components/windows/`);
        return Promise.resolve(null); // Return a resolved Promise with null
    }
}

// Register a window component
function registerWindow(window) {
    const { id, title, active } = window;
    resolveComponentById(id).then((component) => {
        if (component) {
            if (!state.windows.some((w) => w.id === id)) {
                state.windows.push({
                    id,
                    title,
                    component,
                });
                if (active) {
                    state.activeWindow = id; // Set the first registered window as active
                }
            }
        }
    });
}

function setActiveWindow(windowId) {
    state.activeWindow = windowId;
}

export function useWindowManager() {
    return {
        state,
        registerWindow,
        setActiveWindow,
    };
}

import { reactive, markRaw, computed } from 'vue';

class WindowManager {
    constructor() {
        this.state = reactive({
            windows: [],
            activeWindow: null,
            direction: 'up',
        });
        this.modules = import.meta.glob('@/components/windows/*.vue');
    }

    async initializeWindows() {
        const moduleEntries = Object.entries(this.modules);

        await Promise.all(
            moduleEntries.map(async ([path, loader]) => {
                const module = await loader();
                const id = path.match(/\/([^/]+)\.vue$/)[1];
                const title = module.default?.title || id;
                const startActive = module.default?.startActive || false;

                if (!this.state.windows.find((w) => w.id === id)) {
                    this.state.windows.push({
                        id,
                        title,
                        component: markRaw(module.default),
                    });

                    if (startActive) {
                        this.state.activeWindow = id;
                    }
                }
            })
        );

        if (!this.state.activeWindow && this.state.windows.length > 0) {
            this.state.activeWindow = this.state.windows[0].id;
        }

        this.state.windows.sort((a, b) => a.id.localeCompare(b.id));
    }

    moveUp() {
        this.state.direction = 'up';
        this.state.activeWindow = this.state.windows[this.getCircularIndex(this.activeIndex - 1)].id;
    }

    moveDown() {
        this.state.direction = 'down';
        this.state.activeWindow = this.state.windows[this.getCircularIndex(this.activeIndex + 1)].id;
    }

    get totalWindows() {
        return computed(() => this.state.windows.length).value;
    }

    get activeIndex() {
        return computed(() =>
            this.state.windows.findIndex((window) => window.id === this.state.activeWindow)
        ).value;
    }

    get windows() {
        return this.state.windows;
    }

    get activeWindow() {
        return this.state.activeWindow;
    }

    get direction() {
        return this.state.direction
    }

    getCircularIndex(index) {
        return (index + this.totalWindows) % this.totalWindows;
    }
}

const manager = new WindowManager();

export default manager;

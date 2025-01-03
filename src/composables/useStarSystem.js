import init, { StarSystem } from '@/wasm/wasm.js';
import { ref } from 'vue';

const canvas = ref(null);
const options = ref({
    log_level: 'warn',
    star_count: 0,
    clear_color: '#252525',
    use_advanced: true,
    star_size: 0.8,
});
const starSystem = ref(null);

let targetStarCount = 8000;
const starIncrement = 1000;

export function useStarSystem() {
    async function setup() {
        await init();
        if (window.innerWidth < 900) {
            targetStarCount = 2000;
            options.value.star_size = 1.5;
        }

        canvas.value = document.getElementById('background');
        canvas.value.width = window.innerWidth;
        canvas.value.height = window.innerHeight;

        const stars = new StarSystem(canvas.value, options.value);
        await stars.init();

        window.addEventListener('resize', () => {
            canvas.value.width = window.innerWidth;
            canvas.value.height = window.innerHeight;
            stars.resize(canvas.value);
        });

        starSystem.value = stars;

        let counter = targetStarCount/starIncrement;
        function incrementStars() {
            counter -= 1;
            starSystem.value.add_stars(starIncrement);
            if (counter > 0) {
                setTimeout(incrementStars, 1000);
            }
        }

        let lastTime = performance.now();
        function render() {
            const currentTime = performance.now();
            const deltaTime = (currentTime - lastTime) / 1000; // Convert to seconds
            lastTime = currentTime;
            starSystem.value.update_and_render(deltaTime);
            requestAnimationFrame(render);
        }

        incrementStars();
        render();
    }

    function addStars(count) {
        starSystem.value.add_stars(count);
    }

    function removeStars(count) {
        starSystem.value.remove_stars(count);
    }

    return {
        canvas,
        options,
        starSystem,
        setup,
        addStars,
        removeStars,
    };
}

import { useStorage } from '@vueuse/core';

const linkHistory = useStorage('linkHistory', [], localStorage);

export function useLink() {
    function openLink(url, data = {}) {
        const index = linkHistory.value.findIndex(item => item.url === url);
        if (index !== -1) linkHistory.value.splice(index, 1);
        linkHistory.value.unshift({ url, ...data });
        if (linkHistory.value.length > 12) linkHistory.value.pop();
        window.open(url, '_blank', 'noopener,noreferrer');
    }

    function clearHistory() {
        linkHistory.value = [];
    }

    return {
        linkHistory,
        openLink,
        clearHistory,
    }
}

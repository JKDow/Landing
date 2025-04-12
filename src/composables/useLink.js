import { useStorage } from '@vueuse/core';

const linkHistory = useStorage('linkHistory', [], localStorage);

export function useLink() {
    function newTab(url) {
        if (!url) return;
        window.open(url, '_blank', 'noopener,noreferrer');
    }

    function openLink(url, data = {}) {
        const index = linkHistory.value.findIndex(item => item.url === url);
        if (index !== -1) linkHistory.value.splice(index, 1);
        linkHistory.value.unshift({ url, ...data });
        if (linkHistory.value.length > 12) linkHistory.value.pop();
        newTab(url);
    }

    function openQuickLink(link) {
        const index = linkHistory.value.findIndex(item => item.url === link?.url);
        if (index === -1) return;
        const linkItem = linkHistory.value.splice(index, 1)[0];
        linkHistory.value.unshift(linkItem);
        newTab(linkItem.url);
    }

    function clearHistory() {
        linkHistory.value = [];
    }

    return {
        linkHistory,
        newTab,
        openLink,
        openQuickLink,
        clearHistory,
    }
}

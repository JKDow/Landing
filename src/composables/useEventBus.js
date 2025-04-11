const eventTarget = new EventTarget();

export function useEventBus() {
    function emit(event, data = null) {
        eventTarget.dispatchEvent(new CustomEvent(event, { data }));
    }

    function on(event, callback) {
        eventTarget.addEventListener(event, callback);
    }

    function off(event, callback) {
        eventTarget.removeEventListener(event, callback);
    }

    return {
        emit,
        on,
        off,
    };
}

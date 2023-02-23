export function form_event_to_object<T>(event: SubmitEvent): T {
    const iterable = new FormData(event.target as HTMLFormElement).entries()
    return collect(iterable)
}

function collect<T,U>(iter: IterableIterator<[string, T]>) { 
    const object: Record<string, T> = {};
    for (const [key, value] of iter) {
        object[key] = value
    }
    return object as U
}
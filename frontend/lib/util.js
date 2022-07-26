// converts seconds to times such as 1h
export function secondsToTimeString(sec) {
    const days = Math.floor(sec / 86400);
    const hours = Math.floor(sec / 3600);
    const minutes = Math.floor((sec - (hours * 3600)) / 60);
    const seconds = sec - (hours * 3600) - (minutes * 60);

    if (days > 0) return `${days}d`;
    else if (hours > 0) return `${hours}h`;
    else if (minutes > 0) return `${minutes}m`;
    else return `${seconds}s`;
}

const map = {
    d: 24 * 60 * 60,
    h: 60 * 60,
    m: 60,
    s: 1
};

export function intervalToSeconds(str) {
    let secs = 0;
    for (const match of str.match(/^(\d+d)?(\d+h)?(\d+m)?(\d+s)?$/).slice(1, 5))
        if (match != undefined)
            secs += map[match[match.length - 1]] * match.slice(0, -1);
    return secs;
}

export function secondsToInterval(sec) {
    const days = Math.floor(sec / 86400);
    const hours = Math.floor(sec / 3600);
    const minutes = Math.floor(sec / 60);
    const seconds = sec;

    if (seconds % 86400 == 0) return `${days}d`;
    else if (seconds % 3600 == 0) return `${hours}h`;
    else if (seconds % 60 == 0) return `${minutes}m`;
    else return `${seconds}s`;
}

export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

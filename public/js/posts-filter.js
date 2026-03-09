(function () {
    const params = new URLSearchParams(window.location.search);
    const requestedTag = params.get("tag");

    if (!requestedTag) {
        return;
    }

    const normalizedTag = requestedTag.trim().toLowerCase();
    if (!normalizedTag) {
        return;
    }

    const items = document.querySelectorAll(".posts-page-item");
    const emptyState = document.querySelector(".posts-page__empty");
    let visibleCount = 0;

    items.forEach((item) => {
        const tagValue = item.getAttribute("data-tags") || "";
        const tags = tagValue
            .split("|")
            .map((tag) => tag.trim().toLowerCase())
            .filter(Boolean);

        const matches = tags.includes(normalizedTag);
        item.hidden = !matches;
        if (matches) {
            visibleCount += 1;
        }
    });

    if (emptyState) {
        emptyState.hidden = visibleCount !== 0;
    }
})();

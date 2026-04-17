const dialog = document.querySelector(".qr-share__dialog");
const openBtn = document.querySelector(".qr-share__open");
const closeBtn = document.querySelector(".qr-share__close");

openBtn.addEventListener("click", () => dialog.showModal());
closeBtn.addEventListener("click", () => dialog.close());
dialog.addEventListener("click", (e) => {
    if (e.target === dialog) dialog.close();
});

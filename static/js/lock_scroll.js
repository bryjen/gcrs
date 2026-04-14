window.ScrollLock = (() => {
  let locked = false;
  let scrollY = 0;
  let unlockTimer = null;

  return {
    lock() {
      if (locked) return;
      locked = true;
      scrollY = window.scrollY;
      document.body.style.overflow = "hidden";
      document.body.style.position = "fixed";
      document.body.style.top = `-${scrollY}px`;
      document.body.style.width = "100%";
    },
    unlock(delay = 0) {
      clearTimeout(unlockTimer);
      unlockTimer = setTimeout(() => {
        if (!locked) return;
        locked = false;
        document.body.style.overflow = "";
        document.body.style.position = "";
        document.body.style.top = "";
        document.body.style.width = "";
        window.scrollTo(0, scrollY);
      }, delay);
    },
  };
})();

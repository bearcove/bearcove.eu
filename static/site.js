(() => {
  const status = document.getElementById('copy-status');
  const fallback = value => {
    const field = document.createElement('textarea');
    field.value = value; field.setAttribute('readonly', '');
    field.style.cssText = 'position:fixed;top:0;left:-9999px';
    document.body.append(field); field.select();
    const ok = document.execCommand('copy'); field.remove();
    if (!ok) throw new Error('Clipboard unavailable');
  };
  document.querySelectorAll('[data-copy]').forEach(button => {
    let reset;
    button.addEventListener('click', async () => {
      clearTimeout(reset);
      try {
        const value = button.dataset.copy;
        if (navigator.clipboard?.writeText) {
          try { await navigator.clipboard.writeText(value); } catch { fallback(value); }
        } else fallback(value);
        button.dataset.state = 'copied'; button.querySelector('span').textContent = 'Copied';
        status.textContent = `${button.getAttribute('aria-label').replace(/^Copy /, '')} copied to clipboard.`;
      } catch {
        button.dataset.state = 'error'; button.querySelector('span').textContent = 'Select';
        const range = document.createRange(); range.selectNodeContents(button.previousElementSibling);
        getSelection().removeAllRanges(); getSelection().addRange(range);
        status.textContent = 'Clipboard is unavailable. The text is selected so you can copy it manually.';
      }
      button.focus({preventScroll:true});
      reset = setTimeout(() => { button.querySelector('span').textContent = 'Copy'; delete button.dataset.state; }, 2500);
    });
  });
})();

+++
title = "Legal / Mentions légales"
+++

<div class="legal-grid">

<div class="legal-card">
<h3 class="legal-heading"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="7" width="20" height="14" rx="2" ry="2"/><path d="M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16"/></svg> Company</h3>
<p><strong>bearcove SARL</strong><br>
French limited liability company (SARL)<br>
Share capital: €10,000</p>
</div>

<div class="legal-card">
<h3 class="legal-heading"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/><polyline points="10 9 9 9 8 9"/></svg> Registration</h3>
<div class="reg-row">
<span class="reg-label">RCS Lyon</span>
<span class="copyable" data-value="934149618">934 149 618<button class="copy-btn" title="Copy"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg></button></span>
</div>
<div class="reg-row">
<span class="reg-label">VAT</span>
<span class="copyable" data-value="FR41934149618">FR41 934 149 618<button class="copy-btn" title="Copy"><svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"/><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/></svg></button></span>
</div>
</div>

<div class="legal-card">
<h3 class="legal-heading"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/><circle cx="12" cy="10" r="3"/></svg> Registered office</h3>
<p>bearcove SARL<br>
4 Quai Jean Moulin<br>
c/o La Cordée SAS<br>
69001 Lyon, France</p>
</div>

<div class="legal-card">
<h3 class="legal-heading"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg> Publication director</h3>
<p>Amos Wenger, Manager (Gérant)<br>
<a href="mailto:hi@bearcove.eu">hi@bearcove.eu</a></p>
</div>

<div class="legal-card legal-card-full">
<h3 class="legal-heading"><svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="2" width="20" height="8" rx="2" ry="2"/><rect x="2" y="14" width="20" height="8" rx="2" ry="2"/><line x1="6" y1="6" x2="6.01" y2="6"/><line x1="6" y1="18" x2="6.01" y2="18"/></svg> Hosting</h3>
<div class="hosting-grid">
<div class="hosting-entry">
<p><img class="hosting-favicon" src="https://www.hetzner.com/favicon.ico" alt="" width="20" height="20"> <a href="https://www.hetzner.com">Hetzner Online GmbH</a><br>
Industriestr. 25<br>
91710 Gunzenhausen, Germany</p>
</div>
<div class="hosting-entry">
<p><img class="hosting-favicon" src="https://github.githubassets.com/favicons/favicon.svg" alt="" width="20" height="20"> <a href="https://github.com">GitHub, Inc.</a><br>
88 Colin P Kelly Jr St<br>
San Francisco, CA 94107, USA</p>
</div>
</div>
</div>

</div>

<script>
document.querySelectorAll('.copy-btn').forEach(btn => {
    btn.addEventListener('click', () => {
        const value = btn.closest('.copyable').dataset.value;
        navigator.clipboard.writeText(value).then(() => {
            btn.classList.add('copied');
            setTimeout(() => btn.classList.remove('copied'), 1500);
        });
    });
});
</script>

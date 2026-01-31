Teraz dodajmy kod naszego widżetu.

Skopiuj kod poniżej. Upewnij się, że jesteś zalogowany na [fastcomments.com](https://fastcomments.com) 
i w razie potrzeby odśwież tę stronę, aby kod został wstępnie wypełniony danymi Twojego konta; w przeciwnym razie pokaże się kod demonstracyjny.

Skopiujmy teraz kod:

[inline-code-attrs-start title = 'Kod komentarzy Zyro'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    }];
</script>
[inline-code-end]

Wróćmy teraz do kreatora strony i kliknij `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Wprowadź kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Wprowadź kod" />
</div>

### Uwaga!

Ważne: użyj powyższego kodu, a nie fragmentów kodu z innej dokumentacji, ponieważ ten fragment został przygotowany specjalnie dla Zyro.

Powinieneś teraz zobaczyć coś podobnego do poniższego, co wydaje się puste. To oczekiwane. Najedź kursorem myszy na obszar
gdzie powinien znajdować się widżet:

<div class="screenshot white-bg">
    <div class="title">Dodano widżet kodu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodano widżet kodu" />
</div>

Teraz przeciągnij widżet, aby ustawić żądany rozmiar — zobaczysz, jak się pojawia:

<div class="screenshot white-bg">
    <div class="title">Zmień rozmiar</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Zmień rozmiar" />
</div>

...a teraz wyświetl podgląd i zapisz!

---
Dodajmy teraz kod naszego widżetu.

Skopiuj poniższy kod. Upewnij się, że jesteś zalogowany na [fastcomments.com](https://fastcomments.com) 
i przeładuj tę stronę, jeśli nie jesteś, aby kod został wstępnie wypełniony informacjami o Twoim koncie, w przeciwnym razie wyświetli się kod demonstracyjny.

Skopiujmy teraz kod:

[inline-code-attrs-start title = 'Kod komentarzy Hostinger'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    FastCommentsUI(document.getElementById('fastcomments-widget'), {
        tenantId: "demo",
        pageTitle: window.parent.document.title,
        urlId: window.parent.location.href,
        url: window.parent.location.href
    });
</script>
[inline-code-end]

Wróćmy teraz do naszego kreatora stron i kliknij `Enter code`:

<div class="screenshot white-bg">
    <div class="title">Wprowadź kod</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-2.png" alt="Wprowadź kod" />
</div>

### Uwaga!

Ważne jest, aby użyć powyższego kodu, a nie fragmentów kodu z innej dokumentacji, ponieważ ten fragment został przygotowany specjalnie
dla Hostinger.

Powinieneś teraz zobaczyć coś podobnego do poniższego, co wygląda na puste. To jest oczekiwane. Najedź myszką na obszar,
gdzie powinien znajdować się widżet:

<div class="screenshot white-bg">
    <div class="title">Dodano widżet kodu</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-3.png" alt="Dodano widżet kodu" />
</div>

Teraz przeciągnij widżet do żądanego rozmiaru — zobaczysz, jak się wyświetla:

<div class="screenshot white-bg">
    <div class="title">Dopasuj rozmiar</div>
    <img class="screenshot-image" src="/images/installation-guides/hostinger-step-2-add-code-4.png" alt="Dopasuj rozmiar" />
</div>

...a teraz wyświetl podgląd i zapisz!
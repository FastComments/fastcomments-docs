Teraz wygenerujemy dla Ciebie niestandardowy kod FastComments. Użyj poniższego kreatora, aby skonfigurować, jak chcesz, aby FastComments działał na Twojej stronie GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Różne typy okienka komentarzy

Możesz skonfigurować linię `TYPE = 'commenting'`, aby przełączyć używany produkt (na przykład możesz zmienić ją na `live` dla czatu na żywo lub `collab` dla czatu kolaboracyjnego).

### Umieszczanie okienka komentarzy tam, gdzie chcesz

Załóżmy, że chcesz umieścić pola komentarzy w konkretnych częściach strony, a nie w domyślnych miejscach.
Zmień tę linię:

    const TARGET_ELEMENT_ID = ''; // ustaw, aby użyć trybu docelowego elementu div

Na:

    const TARGET_ELEMENT_ID = 'fc_box'; // ustaw, aby użyć trybu docelowego elementu div

Then in the GHL editor, click the "code" button and add where you want the comments to go:

[inline-code-attrs-start title = 'Div FastComments dla GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Różny typ okienka komentarzy na stronę

Załóżmy, że chcesz, aby użytkownicy mogli wyróżniać i dyskutować fragmenty tekstu lub zamiast tego korzystać z interfejsu czatu na żywo.

Najpierw wykonaj powyższe kroki z sekcji „Umieszczanie okienka komentarzy tam, gdzie chcesz”.

Note in that small snippet there's `type="commenting"`.

If you want to enable collab chat for example change type to `type="collab"`.

### Wyświetlaj tylko na wybranych stronach

Jeśli nie ustawisz `TARGET_ELEMENT_ID`, możesz zamiast tego skonfigurować zmienną `VALID_PATTERNS`, aby określić, na których trasach URL mają się wyświetlać komentarze. Domyślnie będą one wyświetlane na stronach, których URL zawiera `/post`.

### Konfigurowanie czatu kolaboracyjnego

Możesz skonfigurować czat kolaboracyjny tak, aby dodawał funkcje współpracy tylko wokół HTML-a wewnątrz określonego obszaru, na przykład załóżmy, że dodałeś powyższy kod w stopce, a następnie dodajesz ten div w treści posta/strony, aby włączyć czat kolaboracyjny:

[inline-code-attrs-start title = 'Czat kolaboracyjny z określoną zawartością'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Wówczas element paragrafu wewnątrz `<div>` będzie miał włączony czat kolaboracyjny, a nic więcej na stronie. Jeśli nie umieścisz żadnej zawartości w `<div>`, wówczas włączy to czat kolaboracyjny dla całej treści posta.

---
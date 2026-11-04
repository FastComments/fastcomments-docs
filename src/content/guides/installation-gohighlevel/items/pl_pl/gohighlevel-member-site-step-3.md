Teraz wygenerujemy Twój niestandardowy kod FastComments. Użyj kreatora poniżej, aby skonfigurować sposób działania FastComments na Twojej stronie GoHighLevel:

[snippet id="gohighlevel-wizard"]

### Różne typy pól komentarzy

Możesz skonfigurować linię `TYPE = 'commenting'`, aby przełączyć używany produkt (na przykład możesz zmienić ją na `live` dla czatu na żywo lub `collab` dla czatu współpracy).

### Umieszczanie pola komentarzy w wybranym miejscu

Załóżmy, że chcesz umieścić pola komentarzy w określonych częściach strony, a nie w domyślnych lokalizacjach.
Zmień tę linię:

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Na:

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Następnie w edytorze GHL kliknij przycisk "code" i dodaj tam, gdzie mają się znaleźć komentarze:

[inline-code-attrs-start title = 'GoHighLevel FastComments Div'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Różny typ pola komentarzy na każdej stronie

Załóżmy, że chcesz, aby użytkownicy mogli wyróżniać i omawiać fragmenty tekstu, albo chcesz użyć interfejsu czatu na żywo.

Najpierw wykonaj powyższe kroki z sekcji "Umieszczanie pola komentarzy w wybranym miejscu".

Zauważ, że w tym małym fragmencie znajduje się `type="commenting"`.

Jeśli chcesz na przykład włączyć czat współpracy, zmień typ na `type="collab"`.

### Wyświetlanie tylko na określonych stronach

Jeśli nie ustawisz `TARGET_ELEMENT_ID`, możesz zamiast tego skonfigurować zmienną `VALID_PATTERNS`, aby określić, na jakich ścieżkach URL mają się wyświetlać komentarze. Domyślnie będą wyświetlać się
na stronach zawierających `/post` w adresie URL.

### Konfiguracja czatu współpracy

Możesz nakazać czatowi współpracy dodawanie funkcji współpracy tylko wokół HTML w określonym obszarze. Na przykład, załóżmy, że
dodajesz powyższy kod stopki, a następnie dodajesz ten div w treści wpisu/strony, aby włączyć czat współpracy:

[inline-code-attrs-start title = 'Czat współpracy z określoną treścią'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Wtedy element akapitu wewnątrz `<div>` będzie miał włączony czat współpracy, a nic innego na stronie. Jeśli nie
umieścisz żadnej treści w `<div>`, czat współpracy zostanie włączony na całej treści wpisu.
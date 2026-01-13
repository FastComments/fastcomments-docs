---
Obiekt `HashTag` reprezentuje tag, który może zostawić użytkownik. Hashtagi mogą być używane do łączenia z zewnętrzną treścią lub do
łączenia ze sobą powiązanych komentarzy.

Struktura obiektu `HashTag` jest następująca:

[inline-code-attrs-start title = 'Struktura obiektu HashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Powinien zaczynać się od "#" lub wybranego znaku. **/
    tag: string
    /** Opcjonalny URL, na który może wskazywać hashtag. Zamiast filtrować komentarze po haśtagu, interfejs przekieruje do tego po kliknięciu. **/
    url?: string
    /** TYLKO DO ODCZYTU **/
    createdAt: string
}
[inline-code-end]

Notes:

- W niektórych punktach końcowych API zobaczysz, że hashtag jest używany w URL. Pamiętaj o kodowaniu wartości URI. Na przykład, `#` powinien być zamiast tego reprezentowany jako `%23`.
- Niektóre z tych pól są oznaczone `READONLY` - są one zwracane przez API, ale nie można ich ustawić.
 
---
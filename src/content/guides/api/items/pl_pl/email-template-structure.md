Obiekt `EmailTemplate` reprezentuje konfigurację niestandardowego szablonu e-mail dla dzierżawcy.

System wybierze używany szablon e-mail na podstawie:

- jego identyfikatora typu, nazywanego `emailTemplateId`. Są to stałe.
- `domain`. Najpierw spróbujemy znaleźć szablon dla domeny, z którą powiązany jest dany obiekt (np. `Comment`), a jeśli nie znajdzie się dopasowanie, spróbujemy znaleźć szablon, gdzie domain jest null lub `*`.

Struktura obiektu `EmailTemplate` wygląda następująco:

[inline-code-attrs-start title = 'Struktura szablonu e-mail'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** TYLKO DO ODCZYTU **/
    createdAt: string
    /** TYLKO DO ODCZYTU **/
    updatedAt: string
    /** TYLKO DO ODCZYTU **/
    updatedByUserId: string
    /** Domena, z którą powinien być powiązany szablon. **/
    domain?: string | '*' | null
    /** Zawartość szablonu e-mail w składni EJS. **/
    ejs: string
    /** Mapowanie nadpisanych kluczy tłumaczeń na wartości dla każdej obsługiwanej lokalizacji. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Obiekt reprezentujący kontekst renderowania szablonu. **/
    testData: object
}
[inline-code-end]

### Uwagi

- Poprawne wartości `emailTemplateId` można pobrać z endpointu `/definitions`.
- Endpoint `/definitions` zawiera również domyślne tłumaczenia i dane testowe.
- Zapisywanie szablonów zakończy się niepowodzeniem, jeśli struktura lub dane testowe są nieprawidłowe.
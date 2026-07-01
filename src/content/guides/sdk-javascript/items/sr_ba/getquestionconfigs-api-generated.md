## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| skip | number | No |  |

## Odgovor

Vraća: [`GetQuestionConfigsResponse1`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfigsResponse1.ts)

## Primjer

[inline-code-attrs-start title = 'Primer getQuestionConfigs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchQuestionConfigs() {
    const tenantId: string = "tenant-9876";

    // Poziv bez opcionog parametra 'skip'
    const configsWithoutSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId);

    // Poziv s opcionim parametrom 'skip'
    const skip: number = 10;
    const configsWithSkip: GetQuestionConfigsResponse1 = await getQuestionConfigs(tenantId, skip);

    console.log(configsWithoutSkip, configsWithSkip);
}
[inline-code-end]
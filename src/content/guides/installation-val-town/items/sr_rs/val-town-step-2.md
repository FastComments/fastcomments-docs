`tenantId: "demo"` је дељени јавни песак. Ради без регистрације, због чега се примери користе, али сви остали који користе FastComments пишу у истим темама и свако их може модерирати. Промените пре него што објавите било шта што вам је важно.

Ваш tenant ID је на [страници за API тајну](https://fastcomments.com/auth/my-account/api-secret).

Tenant ID је јаван и треба да буде у коду у прегледачу. API тајна није, и ништа на овој страници не захтева једну.

## Прочитајте га из променљиве окружења

Val Town валови су јавни у бесплатном слоју, тако да је њихов извор читљив свима. Чувајте све осетљиво у променљивим окружења, читајте их помоћу `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Ово је важније него обично на Val Town из другог разлога: **ремиксовање вал‑а копира кључеве променљивих окружења, али не и њихове вредности.** Тајна сачувана у променљивој окружења не прати ваш вал у налог друге особе. Тајна уписана у датотеку то чини.

Враћање на `"demo"` одржава вал у раду за свакога ко га ремиксује пре него што постави сопствени tenant.

## EU налози

Налог, његови подаци и кључеви живе у једној региони. Ако је ваш креиран на `eu.fastcomments.com`, свака конфигурација виџета такође треба `region: "eu"`, а скрипте се учитавају са `cdn-eu.fastcomments.com`. У супротном оставите оба нетакнута.
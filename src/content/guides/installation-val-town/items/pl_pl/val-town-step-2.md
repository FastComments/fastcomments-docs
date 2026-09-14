`tenantId: "demo"` jest współdzielonym publicznym sandboxem. Działa bez rejestracji, dlatego przykłady go używają, ale wszyscy inni korzystający z FastComments piszą w tych samych wątkach i każdy może je moderować. Przełącz się, zanim opublikujesz cokolwiek, na czym Ci zależy.

Twój identyfikator najemcy znajduje się na [stronie tajnego klucza API](https://fastcomments.com/auth/my-account/api-secret).

Identyfikator najemcy jest publiczny i powinien znajdować się w kodzie przeglądarki. Tajny klucz API nie jest publiczny i nic na tej stronie nie wymaga go.

## Odczytaj go ze zmiennej środowiskowej

Wartości Val Town są publiczne w darmowym planie, więc ich źródło jest dostępne dla wszystkich. Przechowuj wszelkie wrażliwe dane w zmiennych środowiskowych, odczytując je za pomocą `Deno.env.get`:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Ma to większe znaczenie niż zwykle w Val Town z powodu drugiego powodu: **przepisanie wartości (remiksowanie) kopiuje klucze zmiennych środowiskowych, ale nie ich wartości.** Tajny klucz przechowywany w zmiennej środowiskowej nie podąża za Twoją wartością do konta innej osoby. Tajny klucz zapisany w pliku podąża.

Powrót do `"demo"` utrzymuje działanie wartości dla każdego, kto ją przepisze przed ustawieniem własnego najemcy.

## Konta UE

Konto, jego dane i klucze znajdują się w jednym regionie. Jeśli Twoje zostało utworzone na `eu.fastcomments.com`, każda konfiguracja widgetu musi również zawierać `region: "eu"`, a skrypty ładują się z `cdn-eu.fastcomments.com`. W przeciwnym razie pozostaw oba niezmienione.
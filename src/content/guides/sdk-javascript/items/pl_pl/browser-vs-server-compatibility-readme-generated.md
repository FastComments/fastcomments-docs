---
Ten SDK korzysta z **podwójnych punktów wejścia** aby zapewnić optymalną kompatybilność i zapobiec błędom czasu wykonywania:

- **`fastcomments-sdk/browser`** - Wersja bezpieczna dla przeglądarki z natywnym `fetch`
- **`fastcomments-sdk/server`** - Pełna wersja Node.js z obsługą SSO
- **`fastcomments-sdk`** (domyślnie) - Tylko typy, bezpieczne do importowania w dowolnym miejscu
---
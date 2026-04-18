Jeśli zainstalowano FastComments przez integrację Vercel Marketplace, identyfikator najemcy jest dostępny jako zmienna środowiskowa `FASTCOMMENTS_TENANT_ID`. Aby odczytać go po stronie klienta, udostępnij go przez `next.config.js` lub poprzedź go prefiksem `NEXT_PUBLIC_`:

```tsx
<FastComments tenantId={process.env.NEXT_PUBLIC_FASTCOMMENTS_TENANT_ID!} />
```
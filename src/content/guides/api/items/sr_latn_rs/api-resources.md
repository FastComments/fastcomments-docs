### Korišćenje resursa

Treba napomenuti da preuzimanje podataka sa API‑ja računa se kao korišćenje na vašem nalogu.

Svaki resurs će navesti to korišćenje u svom odeljku.

Neki resursi koštaju više za servisiranje od drugih. Svaki endpoint ima fiksnu cenu u kreditima po API pozivu. Za neke endpoint‑e, broj kredita varira u zavisnosti od opcija i veličina odgovora.

API korišćenje možete proveriti na stranici [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) i ažurira se svakih nekoliko minuta.

#### Napomena!

Preporučujemo da prvo pročitate dokumentaciju za Pages, kako biste smanjili zabunu prilikom određivanja koje vrednosti proslediti za `urlId` u Comment API‑ju.

### Webhook‑ovi

Pretplate na webhook‑ove imaju svoj vodič. `POST`, `GET` i `DELETE /api/v1/webhooks`, i `GET /api/v1/webhooks/sample-payloads`, su dokumentovani u odeljku [Managing Subscriptions via API](/guide-webhooks.html#webhooks-api-subscriptions), a strukture događaja u [Webhook Structures](/guide-webhooks.html#webhooks-structures).

---
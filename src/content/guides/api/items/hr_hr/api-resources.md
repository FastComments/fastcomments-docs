### Korištenje resursa

Treba napomenuti da se dohvaćanje podataka s API-ja računa kao korištenje na vašem računu.

Svaki resurs će navesti to korištenje u svojoj sekciji.

Neki resursi koštaju više za posluživanje od drugih. Svaki endpoint ima fiksnu cijenu kredita po API pozivu. Za neke endpointove, broj kredita varira ovisno o opcijama i veličinama odgovora.

Korištenje API-ja možete provjeriti na stranici [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) i ažurira se svakih nekoliko minuta.

#### Napomena!

Preporučujemo da najprije pročitate dokumentaciju za Pages, kako biste smanjili zabunu pri određivanju koje vrijednosti proslijediti za `urlId` u Comment API-ju.

### Webhookovi

Pretplate na webhookove imaju svoj vodič. `POST`, `GET` i `DELETE /api/v1/webhooks`, te `GET /api/v1/webhooks/sample-payloads`, dokumentirani su pod [Managing Subscriptions via API](/guide-webhooks.html#webhooks-api-subscriptions), a strukture događaja pod [Webhook Structures](/guide-webhooks.html#webhooks-structures).

---
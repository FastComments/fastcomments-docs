1. Log ind på FastComments og gå til <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">din LTI 1.3-konfigurationsside</a>.
2. (Valgfrit) Vælg den platform, du forbinder fra i rullemenuen **Platform** - den angiver visningsetiketten, men Auto-detect fungerer fint.
3. Klik på **Generate URL**. En engangs **Registration URL** vises (gyldig i 30 minutter, til enkelt brug).
4. I dit LMS skal du åbne skærmen LTI 1.3 Dynamic Registration og indsætte URL'en i feltet **Tool initiation registration endpoint** (eller tilsvarende). Indsend.
5. Dit LMS ringer tilbage til FastComments, udveksler nøgler og opretter integrationen. Popup'en lukker sig selv, når det er færdigt.
6. Tilbage i FastComments vises den nye konfiguration i tabellen **Existing Configurations**. Værktøjet er nu tilgængeligt i dine LMS-kurser.
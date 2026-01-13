---
Domyślnie każdy użytkownik może przesłać do `5 comments` w tej samej minucie.

To jest śledzone na podstawie user id, anon user id i ip address (hashed).

Można to dostosować bez użycia kodu, na stronie konfiguracji widgetu:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Uwaga: jeśli używasz comment creation API, możesz chcieć przekazać oryginalny adres `ip` użytkownika w żądaniu do naszego backendu, aby rate limiting był stosowany
per user, a nie globalnie dla twojego konta.

---
---
Je wilt de config waarschijnlijk niet inline definiëren als je callbacks enz. doorgeeft. In plaats daarvan wil je de
config definiëren via `computed()`, anders zal telkens wanneer je callback enz. wordt aangeroepen de hele widget opnieuw renderen.
---
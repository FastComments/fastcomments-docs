Alle Metriken, für die Sie ratenlimitiert oder in Rechnung gestellt werden können, werden auf der [Billing Analytics Page](https://fastcomments.com/auth/my-account/analytics/billing) ausgewiesen.

Einige Beispielmetriken, die erfasst werden:

- Page Loads (für den aktuellen Monat)
- API Credits (für den aktuellen Monat)
- New Comments Stored (für den aktuellen Monat)
- Tenant Users
- SSO Users (reguläre SSO-Benutzer ohne Admin-/Moderator-Berechtigungen)
- SSO Admins (SSO-Benutzer mit den Admin-Flags `isAccountOwner` oder `isAdminAdmin` - werden separat von regulären SSO-Benutzern abgerechnet)
- SSO Moderators (SSO-Benutzer mit dem Flag `isCommentModeratorAdmin` - werden separat von regulären SSO-Benutzern abgerechnet)
- Moderators
- Domains

Für die Tarife `Creators` und `Pro` dienen diese Metriken lediglich als Orientierung, um zu wissen, ob Sie die Grenzen Ihres Kontos erreicht haben oder kurz davor sind.

Für den Tarif `Flex` zeigt diese Seite eine Kostenaufschlüsselung der aktuellen Nutzung für die **nächste** Rechnung. So können Sie leicht nachvollziehen, wie Ihnen in Rechnung gestellt wird, bevor es passiert. Frühere Rechnungen finden Sie unter [Billing History](https://fastcomments.com/auth/my-account/billing-history). Wie jede Position gezählt wird und wann die Rechnung abgerechnet wird, wird im [Billing guide](/guide-billing.html#how-flex-usage-is-measured) erklärt.

Die Metriken `Page Loads`, `API Credits` und `New Comments Stored` können bis zu einer Minute veraltet sein. Die anderen Metriken sind in Echtzeit.

Diese Informationen werden für historische Zwecke monatlich für ein Jahr gespeichert oder bis Sie Ihren Tenant löschen, aber die Benutzeroberfläche zeigt nur den aktuellen Zeitraum an.
Die Definition, wie mehrere Benutzer interagieren, und deren Test ist kompliziert. Hier ist die folgende Spezifikation, der wir für die Zugriffskontrolle folgen,
die Sie zum Testen Ihrer Implementierung verwenden können:

    Seite mit null Gruppen-IDs, Benutzer mit null Gruppen-IDs - sollte Zugriff haben.
    Seite mit null Gruppen-IDs, Benutzer mit Gruppen-IDs - sollte Zugriff haben.
    Seite mit Gruppen-IDs, Benutzer mit null Gruppen-IDs - sollte Zugriff haben.
    Seite mit Gruppen-IDs, Benutzer mit leerer Liste - sollte KEINEN Zugriff haben.
    Seite mit Gruppen-IDs, Benutzer mit Gruppen-IDs - Schnittmenge vorhanden - sollte Zugriff haben.
    Seite mit Gruppen-IDs, Benutzer mit Gruppen-IDs - Schnittmenge nicht vorhanden - sollte KEINEN Zugriff haben.
    Seite mit leerer Liste von Gruppen-IDs (niemand hat Zugriff), Benutzer mit null - sollte KEINEN Zugriff haben.
    
    SSO User A = Keine Gruppen-IDs definiert (null = voller Zugriff).
    SSO User B = Keine Gruppen-IDs definiert (null = voller Zugriff).
    A kann @B.
    
    SSO User A = Keine Gruppen-IDs definiert (null = voller Zugriff).
    SSO User B = Gruppen-IDs definiert.
    A kann @B.
    
    SSO User A = Gruppen-IDs definiert.
    SSO User B = Keine Gruppen-IDs definiert (null = voller Zugriff).
    A kann @B.
    
    SSO User A = Gruppen-IDs = [a].
    SSO User B = Gruppen-IDs = [b].
    A kann NICHT @B.
    
    SSO User A = Gruppen-IDs = [a].
    SSO User B = Gruppen-IDs = [a, b].
    A kann @B.
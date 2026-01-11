Voor ontwikkelaars die u misschien geen `Administrators` wilt laten zijn, overweeg het aanmaken van een `Administrator` gebruiker met de volgende machtigingen:

1. Analytics Admin
2. Customizations Admin
3. Data Management Admin
4. Comment Moderation Admin
5. API/SSO Admin

Deze set machtigingen geeft een ontwikkelaar alles wat zij nodig hebben om FastComments in te stellen, evenals de zichtbaarheid in het systeem die nodig is om te verzekeren dat het werkt.

De redenering voor deze machtigingen is als volgt:

1. **Analytics Admin**: Dit kan gebruikt worden om het gebruik van FastComments te debuggen.
2. **Customizations Admin**: Dit is vereist om aangepaste styling op de comment-widget toe te passen.
3. **Data Management Admin**: Dit is vereist om importen en exporten te beheren, en webhooks in te stellen.
4. **Comment Moderation Admin**: Dit is vereist om commentaargegevens te kunnen zien, ten minste tijdens de configuratie.
5. **API/SSO Admin**: Dit stelt hen in staat om de API keys direct van ons platform op te halen. Wij beschouwen
dit als veiliger dan dat een `Administrator` het voor hen kopieert en de API Secret per e-mail verstuurt, wat
   misschien niet erg veilig is.
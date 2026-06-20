### Maven

Tilføj Repsy-arkivet til dit projekts POM:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Tilføj derefter de afhængigheder, du har brug for:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Tilføj Repsy-arkivet til din build.gradle-fil:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Bibliotekets indhold

Dette bibliotek indeholder tre moduler. Den genererede API-klient, det centrale Java-bibliotek som indeholder håndskrevne værktøjer
til at gøre arbejdet med API'et lettere, og `pubsub`-modulet som er et bibliotek til abonnement på ændringsfeeds.

- [Dokumentation for API-klientbiblioteket](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentation for Core-biblioteket, inklusive SSO-eksempler](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentation for PubSub-biblioteket](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Offentlige vs. Sikrede API'er

For API-klienten er der tre klasser, `DefaultApi`, `PublicApi`, og `ModerationApi`. `DefaultApi` indeholder metoder, der kræver din API-nøgle, og `PublicApi` indeholder metoder
der kan kaldes direkte fra en browser/mobile enhed/etc uden autentificering.

`ModerationApi` driver moderatorens kontrolpanel. Den indeholder metoder til kommentarmoderation (liste, optælling, søgning, logfiler, og eksport), moderatorhandlinger (fjern/gendan,
flag, sæt gennemgang/spam/godkendelsesstatus, stemmer, og genåbn/luk tråd), udelukkelser (udeluk fra at kommentere, fortryd en udelukkelse, forud-udelukkelsesresuméer, udelukkelsesstatus og præferencer, og antal udelukkede brugere),
og badges & tillid (tildel/fjern et badge, manuelle badges, hent/sæt tillidsfaktor, og brugerens interne profil). Hver `ModerationApi`-metode accepterer en `sso`-parameter så kaldet kan være
udført på vegne af en SSO-autentificeret moderator.
### Maven

Dodajte Repsy repozitorij v POM datoteko vašega projekta:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Nato dodajte potrebne odvisnosti:

```xml
<dependencies>
    <!-- API odjemalec -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Jedrna knjižnica (vključuje SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub knjižnica (za dogodke v živo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Dodajte Repsy repozitorij v datoteko build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API odjemalec
    implementation "com.fastcomments:client:2.0.0"
    
    // Jedrna knjižnica (vključuje SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub knjižnica (za dogodke v živo)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Ta knjižnica vsebuje tri module. Generirani API odjemalec, jedrna Java knjižnica, ki vsebuje ročno napisane pripomočke za lažje delo z API-jem, in modul `pubsub`, ki je knjižnica za naročanje na tokove sprememb.

- [Dokumentacija knjižnice API odjemalca](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Dokumentacija jedrne knjižnice, vključno z primeri SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Dokumentacija PubSub knjižnice](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Za API odjemalca obstajajo tri razrede, `DefaultApi`, `PublicApi` in `ModerationApi`. `DefaultApi` vsebuje metode, ki zahtevajo vaš API ključ, medtem ko `PublicApi` vsebuje metode, ki jih je mogoče poklicati neposredno iz brskalnika/mobilne naprave/ipd. brez preverjanja pristnosti.

`ModerationApi` poganja nadzorno ploščo moderatorja. Vsebuje metode za moderiranje komentarjev (seznam, štetje, iskanje, dnevniki in izvoz), moderacijske ukrepe (odstrani/obnovi, označi, nastavi status pregleda/spama/odobritve, glasovi in znova odpri/zaključi nit), prepovedi (prepoved komentiranja, prekliči prepoved, povzetki pred prepovedjo, status prepovedi in nastavitve ter število prepovedanih uporabnikov) ter značke in zaupanje (podeli/odstrani značko, ročne značke, pridobi/nastavi faktor zaupanja in notranji uporabniški profil). Vsaka metoda `ModerationApi` sprejme parameter `sso`, kar omogoča, da se klic izvede v imenu moderatorja, avtenticiranega prek SSO.
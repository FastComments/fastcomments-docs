### Maven

Додајте Repsy репозиторијум у POM вашег пројекта:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Затим додајте зависности које су вам потребне:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
    </dependency>
</dependencies>
```

### Gradle

Додајте Repsy репозиторијум у build.gradle датотеку:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API клијент
    implementation "com.fastcomments:client:0.0.2"
    
    // Core библиотека (укључује SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // PubSub библиотека (за уживо догађаје)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Library Contents

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане помоћне алате који олакшавају рад са API-јем, и `pubsub` модул који је библиотека за претплату на фидове промена.

- [Документација API клијент библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација Core библиотеке, укључујући примјере SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клијент постоје две класе, `DefaultApi` и `PublicApi`. `DefaultApi` садржи методе које захтијевају ваш API кључ, а `PublicApi` садржи API позиве које је могуће извршити директно из прегледача/мобилног уређаја/итд. без аутентификације.
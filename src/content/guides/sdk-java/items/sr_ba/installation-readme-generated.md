### Maven

Додајте Repsy репозиториј у POM вашег пројекта:

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

Додајте Repsy репозиториј у ваш build.gradle фајл:

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

### Library Contents

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане алате који олакшавају рад са API-јем, и модул `pubsub` који је библиотека за претплату на токове промјена.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Core Library Docs, Including SSO Examples](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [PubSub Library Docs](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

За API клијента, постоје три класе, `DefaultApi`, `PublicApi`, и `ModerationApi`. `DefaultApi` садржи методе које захтијевају ваш API кључ, а `PublicApi` садржи методе које се могу позвати директно из претраживача/мобилног уређаја/итд. без аутентификације.

`ModerationApi` омогућава рад контролне табле модератора. Садржи методе за модерацију коментара (листање, бројање, претрага, логови и извоз), акције модерације (уклони/поврати, пријави, постави статус прегледа/спам/одобрења, гласови, и поновно отварање/затварање нити), бановања (забрана коментарисања, поништавање забране, преглед прије забране, статус и преференције забране, и број забрањених корисника), и ознаке & повјерење (додијели/уклони значку, ручне значке, доби/постави фактор повјерења, и интерни профил корисника). Свака метода `ModerationApi` прихвата `sso` параметар тако да се позив може извршити у име модератора који је аутентификован преко SSO.
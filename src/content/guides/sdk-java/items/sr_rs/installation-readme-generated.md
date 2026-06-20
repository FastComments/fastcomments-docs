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

Додајте Repsy репозиторијум у ваш build.gradle фајл:

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

### Садржај библиотеке

Ова библиотека садржи три модула. Генерисани API клијент, основна Java библиотека која садржи ручно написане алате
који олакшавају рад са API-јем, и модул `pubsub` који је библиотека за претплату на фид измена.

- [Документација за API клијент библиотеку](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Документација основне библиотеке, укључујући примере SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Документација PubSub библиотеке](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Јавни у односу на обезбеђене API-је

За API клијента постоје три класе, `DefaultApi`, `PublicApi`, и `ModerationApi`. `DefaultApi` садржи методе које захтевају ваш API кључ, а `PublicApi` садржи методе
које се могу позивати директно из прегледача/мобилног уређаја/итд. без аутентификације.

`ModerationApi` покреће контролну таблу модератора. Садржи методе за модерацију коментара (листање, бројање, претрага, логови, и извоз), радње модерације (уклони/врати,
означи/пријави, подеси статус за преглед/спам/одобрење, гласови, и поново отвори/затвори нит), забране (забрана коментарисања, поништавање забране, прегледи пре забране, статус и подешавања забране, и бројање забрањених корисника),
и ознаке и поверење (додели/уклони ознаку, ручне ознаке, добиј/постави фактор поверења, и интерни профил корисника). Сваки метод `ModerationApi` прихвата параметар `sso` тако да позив може бити
извршен у име модератора аутентификованог преко SSO.
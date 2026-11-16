Dodajte FastComments SDK v datoteko `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Prepričajte se, da imate v datoteki `settings.gradle.kts` vašega projekta Repsy repozitorij:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // drugi repozitoriji...
    }
}
```
Dodajte FastComments SDK v datoteko `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:2.0.0")
}
```

Prepričajte se, da imate v datoteki `settings.gradle.kts` vašega projekta repozitorij Repsy:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```
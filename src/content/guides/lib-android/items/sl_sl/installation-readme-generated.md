Dodajte FastComments SDK v datoteko `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Prepričajte se, da imate repozitorij Repsy v datoteki `settings.gradle.kts` vašega projekta:

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
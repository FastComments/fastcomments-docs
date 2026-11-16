Dodajte FastComments SDK u `build.gradle.kts` datoteku vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Proverite da imate Repsy repozitorijum u `settings.gradle.kts` datoteci vašeg projekta:

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
Dodajte FastComments SDK u datoteku `build.gradle.kts` svoje aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:sdk:0.0.1")
}
```

Provjerite imate li Repsy spremište u datoteci `settings.gradle.kts` vašeg projekta:

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
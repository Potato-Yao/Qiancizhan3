/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'

// Components
import App from './App.vue'
import { createI18n } from 'vue-i18n'
import enLocale from './locales/en.json'
import zhLocale from './locales/zh.json'

const i18n = createI18n({
    legacy: false,
    locale: 'zh',
    messages: {
        en: enLocale,
        zh: zhLocale
    }
})

// Composables
import { createApp } from 'vue'

const app = createApp(App)

registerPlugins(app)

app.use(i18n)

app.mount('#app')

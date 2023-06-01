import './assets/main.css'

import { createApp } from 'vue'
import Main from './Main0.vue'

const app = createApp(Main).mount('#app')

app.config.errorHandler = (err) => {
    console.log(err)
}
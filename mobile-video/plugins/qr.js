import Vue from 'vue'
import VueQRCodeComponent from 'vue-qrcode-component'
import VueQrcodeReader from 'vue-qrcode-reader'

Vue.component('qr-code', VueQRCodeComponent)
Vue.use(VueQrcodeReader)

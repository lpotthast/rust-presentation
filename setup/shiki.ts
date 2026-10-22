import { defineShikiSetup } from '@slidev/types'
import monokaiPro from './monokai-pro.json'

// Color scheme based on Monokai Pro, so live code matches IDE screenshots.
export default defineShikiSetup(() => ({
  themes: {
    dark: monokaiPro as any,
    light: monokaiPro as any,
  },
}))

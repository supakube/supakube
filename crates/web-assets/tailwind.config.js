/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../web-pages/**/*.rs"],
  /** We need this because we use daisy-rsx library */
  safelist: [
    "text-center",
    "z-[1]",
    "w-52",
    "rounded-box",
    "bg-base-100",
    "shadow",
    {
      pattern: /avatar*|alert*|modal*|btn*|menu*|dropdown*|badge*|card*|input*|select*|textarea*|label*|tab*|tooltip*|flex*|text*|overflow*/
    }
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require("daisyui"),
    require('@tailwindcss/typography')
  ],
  /** We can extend the theme at style it as we wish */
  daisyui: {
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
        },
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
        }
      },
    ],
  }
}


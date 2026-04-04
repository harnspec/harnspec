import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'HarnSpec',
  tagline: '专为 AI 协作设计的轻量级规范',
  favicon: 'favicon.ico',

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: 'https://harnspec.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'harnspec', // Usually your GitHub org/user name.
  projectName: 'harnspec.github.io', // Usually your repo name.

  onBrokenLinks: 'throw',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'zh-Hans',
    locales: ['zh-Hans', 'en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl:
            'https://github.com/harnspec/harnspec/tree/main/docs-site/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],

  themeConfig: {
    // Replace with your project's social card
    image: 'img/social-card.png',
    colorMode: {
      respectPrefersColorScheme: true,
    },
    navbar: {
      title: 'HarnSpec',
      logo: {
        alt: 'HarnSpec Logo',
        src: 'img/logo-with-bg.svg',
        srcDark: 'img/logo-dark-bg.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'guideSidebar',
          position: 'left',
          label: '指南',
        },
        {
          type: 'docSidebar',
          sidebarId: 'referenceSidebar',
          position: 'left',
          label: '参考',
        },

        {
          href: 'https://harnspec.github.io/',
          label: 'Web 应用',
          position: 'left',
        },
        {
          type: 'localeDropdown',
          position: 'right',
        },
        {
          href: 'https://github.com/harnspec/harnspec',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: '文档',
          items: [
            {
              label: '快速开始',
              to: '/docs/guide/getting-started',
            },
            {
              label: 'CLI 参考',
              to: '/docs/reference/cli',
            },
            {
              label: 'AI 集成',
              to: '/docs/guide/usage/advanced-features/agent-configuration',
            },
          ],
        },
        {
          title: '社区',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/harnspec/harnspec',
            },
            {
              label: '问题反馈',
              href: 'https://github.com/harnspec/harnspec/issues',
            },
            {
              label: '讨论区',
              href: 'https://github.com/harnspec/harnspec/discussions',
            },
          ],
        },
        {
          title: '更多',
          items: [

            {
              label: '参与贡献',
              href: 'https://github.com/harnspec/harnspec/blob/main/CONTRIBUTING.md',
            },
            {
              label: '更新日志',
              href: 'https://github.com/harnspec/harnspec/blob/main/CHANGELOG.md',
            },
            {
              label: 'npm 包',
              href: 'https://www.npmjs.com/package/harnspec',
            },
          ],
        },
      ],
      copyright: `版权所有 © ${new Date().getFullYear()} HarnSpec. 使用 Docusaurus 构建。`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
    },
  } satisfies Preset.ThemeConfig,

  markdown: {
    mermaid: true,
  },


  themes: ['@docusaurus/theme-mermaid'],

  plugins: [
    function webpackPlugin(context, options) {
      return {
        name: 'webpack-plugin',
        configureWebpack(config, isServer, utils) {
          return {
            resolve: {
              fallback: {
                'vscode-languageserver-types': false,
                'vscode-jsonrpc': false,
                'vscode-languageserver-textdocument': false,
              },
            },
          };
        },
      };
    },
  ],
};

export default config;

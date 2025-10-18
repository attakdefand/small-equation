# Deployment Instructions

This document provides instructions on how to deploy the web demos to GitHub Pages.

## Automatic Deployment

The web demos are automatically deployed to GitHub Pages whenever changes are pushed to the `main` branch. The deployment workflow is defined in `.github/workflows/deploy-web-demo.yml`.

## Manual Deployment

To manually trigger a deployment:

1. Go to your repository on GitHub: https://github.com/attakdefand/small-equation
2. Click on the "Actions" tab
3. In the left sidebar, click on "Deploy Web Demo to GitHub Pages"
4. Click the "Run workflow" button
5. Select the branch (usually `main`)
6. Click "Run workflow"

## Accessing the Deployed Demos

Once deployed, the demos will be available at:
- Main page: https://attakdefand.github.io/small-equation/
- WASM demo: https://attakdefand.github.io/small-equation/wasm-demo.html
- Basic demo: https://attakdefand.github.io/small-equation/basic-demo.html

## Setting up GitHub Pages (If Not Already Done)

If GitHub Pages is not yet configured for your repository:

1. Go to your repository on GitHub: https://github.com/attakdefand/small-equation
2. Click on "Settings" tab
3. In the left sidebar, click on "Pages"
4. Under "Source", select "GitHub Actions"
5. Click "Save"

## Troubleshooting

If the deployment fails:

1. Check the Actions tab for error messages
2. Ensure all code quality checks pass before deployment
3. Verify that the WASM build is working locally:
   ```
   cd wasm/minimal
   wasm-pack build --target web --out-dir pkg
   ```
4. Check that all necessary files are included in the deployment

## Deployment Process Details

The deployment workflow performs the following steps:

1. Checks out the code
2. Installs Rust toolchain
3. Adds WASM target
4. Installs wasm-pack
5. Builds the WASM package
6. Prepares deployment files:
   - Copies WASM demo files
   - Copies basic web demo files
   - Creates an index page linking to both demos
7. Uploads the artifact
8. Deploys to GitHub Pages

The deployment includes both the WASM-based interactive calculator and the basic static interface demo.
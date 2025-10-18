# Branch Strategy

This document outlines the branching strategy for the Trillion Dollar Equation project.

## Branch Overview

### 1. Main Branches

#### `main`
- **Purpose**: Production-ready code
- **Protection**: Protected branch with required reviews and checks
- **Deployment**: Automatically deployed to production
- **Merge Rules**: 
  - Only from `develop` or `hotfix/*` branches
  - Requires approval from at least 1 reviewer
  - All CI checks must pass
  - Tag must follow semantic versioning (v1.2.3)

#### `develop`
- **Purpose**: Main development branch
- **Protection**: Protected branch with required checks
- **Deployment**: Automatically deployed to staging environment
- **Merge Rules**:
  - Only from `feature/*` branches
  - All CI checks must pass

### 2. Supporting Branches

#### `feature/*`
- **Purpose**: New features and enhancements
- **Naming Convention**: `feature/short-description` (e.g., `feature/wasm-enhancements`)
- **Branch From**: `develop`
- **Merge Back To**: `develop`
- **Lifetime**: Exists until feature is complete and merged

#### `hotfix/*`
- **Purpose**: Urgent production fixes
- **Naming Convention**: `hotfix/short-description` (e.g., `hotfix/critical-bug`)
- **Branch From**: `main`
- **Merge Back To**: `main` and `develop`
- **Lifetime**: Exists until fix is complete and merged

## Workflow

### Feature Development
1. Create feature branch from `develop`: `git checkout -b feature/feature-name develop`
2. Develop feature with regular commits following conventional commits format
3. Push feature branch to remote: `git push origin feature/feature-name`
4. Create Pull Request from feature branch to `develop`
5. After review and all CI checks pass, merge to `develop`

### Release Process
1. Create release branch from `develop`: `git checkout -b release/v1.0.0 develop`
2. Perform final testing and bug fixes
3. Update version in Cargo.toml to match release tag
4. Merge release branch to `main`: `git checkout main && git merge release/v1.0.0`
5. Tag the release: `git tag -a v1.0.0 -m "Release v1.0.0"`
6. Merge release branch to `develop`: `git checkout develop && git merge release/v1.0.0`
7. Delete release branch: `git branch -d release/v1.0.0`

### Hotfix Process
1. Create hotfix branch from `main`: `git checkout -b hotfix/hotfix-name main`
2. Implement fix
3. Push hotfix branch to remote: `git push origin hotfix/hotfix-name`
4. Create Pull Request from hotfix branch to `main`
5. After review and all CI checks pass, merge to `main`
6. Merge hotfix to `develop`: `git checkout develop && git merge hotfix/hotfix-name`
7. Delete hotfix branch: `git branch -d hotfix/hotfix-name`

## Code Quality Checks

All branches are protected by automated code quality checks:

### Security
- Dependency security audit with `cargo-audit`
- Vulnerability scanning for all dependencies
- Check for sensitive data in code

### Code Style
- Formatting check with `rustfmt`
- Linting with `clippy`
- Unused dependencies check with `cargo-machete`
- Outdated dependencies check with `cargo-outdated`

### Testing
- Unit tests execution
- Integration tests
- Documentation tests
- Benchmark tests
- Code coverage analysis with `cargo-tarpaulin`

### Build Verification
- Compilation check
- WASM build verification
- Documentation generation
- Cross-platform compatibility check
- Minimum supported Rust version (MSRV) check

### Code Review
- Automated code review with `reviewdog`
- Conventional commit message validation
- Large file detection
- Code duplication check

## Branch Protection Rules

### `main` Branch
- Require pull request reviews before merging
- Require status checks to pass before merging
- Require branches to be up to date before merging
- Include administrators in restrictions
- Prevent direct pushes (must use PRs)
- Require linear history

### `develop` Branch
- Require status checks to pass before merging
- Require branches to be up to date before merging
- Prevent direct pushes (must use PRs)

### Feature Branches
- Continuous integration checks on all pushes
- Automated code review on pull requests
- Conventional commit validation

## Best Practices

1. **Always create a feature branch** for new work
2. **Keep feature branches short-lived** to minimize merge conflicts
3. **Write descriptive commit messages** following conventional commits
4. **Run tests locally** before pushing changes
5. **Keep pull requests small** and focused on a single feature
6. **Update from `develop` regularly** to stay in sync
7. **Delete branches after merging** to keep repository clean
8. **Use meaningful branch names** that describe the work being done
9. **Follow the established naming conventions** for all branches
10. **Ensure all CI checks pass** before requesting review

## Emergency Procedures

### Critical Security Issue
1. Immediately create a hotfix branch from `main`
2. Implement fix and push to remote
3. Create emergency pull request with high priority label
4. Request immediate review from security team
5. After approval, merge immediately and deploy

### Production Outage
1. Identify root cause
2. Create hotfix branch from `main`
3. Implement fix
4. Create pull request with "URGENT" label
5. Request immediate review
6. After approval, merge and deploy

## GitHub Actions Workflows

The repository uses several GitHub Actions workflows to ensure code quality and safety:

1. **Code Quality Checks** (`code-quality.yml`): Comprehensive checks including security, formatting, linting, testing, and build verification
2. **Branch Protection** (`branch-protection.yml`): Enforces branch rules and prevents unsafe operations
3. **Release** (`release.yml`): Automates the release process with validation steps
4. **Continuous Integration** (`continuous-integration.yml`): Runs basic checks on all branches
5. **Code Review** (`code-review.yml`): Automated code review and analysis

All workflows are designed to fail fast if any quality or safety checks fail, preventing problematic code from being merged.
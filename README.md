# Goals
- Improve source code clarity, correctness, performance, robustness, modifiability, and security.
- Reduce dependencies (npm packages).
- Reduce repository content by 50%:
    - 20% unneeded documentation and configuration
    - 30% lines of code (10% styled components, 5% function duplication, 5% mocks/tests, 10% barrel files / "commented out" code blocks / dead code)

# Strategy
- We consider the value and costs involved.
- We evaluate the return on investment.
- We clarify what we mean by the term Technical Debt as related to long term outcomes
- We do "surgery" to the code - we "put it under" (the code does not run for a period of time), and there is calculated risk involved in the process. The problems are deemed serious enough to warrant this risk.
- We plan ahead what steps we will take during this period, and execute quickly and carefully in the defined time set.
- We look forward to a more positive state at the other side of the process.
- When complete, we assess what was done well and what could have been done better, and if this strategy can be replicated in other parts of the organization.
- Ongoing improvement is maintained going forward. Changes should be continuous and gradual rather than abrupt.
- In ROI consideration, there are risks and advantages associated with each improvement)

# Process methods
- Do code modifications without touching the business logic
- Do code transforms programmatically if possible
- Make use of editor tools and extensions to refactor automatically when possible

# Reorganize source, delete unneeded code and documentation
- Remove top level directories that are not a part of source code
- Move everything that is js or jsx to one file, and all .scss into one file, with the "src/" path at "top" of every file for reference
- Move translations to top level src directory
- Remove all: mocks, tests, .stories (storybook), cypress tests
- Remove all import and export statements that are not npm dependencies (they should be available in the file scope, since the whole FE is one file)
- Remove all babel and webpack dependencies

# Compile
- Install Parcel bundler (or similar minimal-config bundler; Snowpack)
- Make the code compile - fix runtime errors

# Code quality improvements
- Remove all commented out code blocks (intentionally dead code)
- Remove some JSDoc comments if they are verbose or not helpful
- Remove all dependencies that are unused from package.json
- Update all dependencies (to latest major versions, including breaking changes)
- Remove script injection using `.innerHTML` and find appropriate package(s) on npm
- Remove all console.log() statements

# New libraries
- TypeScript
- Tailwind CSS
- UUID (proper random ID generation)

# Libraries to remove
- Styled components (use Tailwind)
- Redux (use React.Context)
- RxJS (use React.Context)
- PropTypes (use TypeScript)

- Move class components to hooks
- Remove all *dead* code (TS will really show this!)
- Remove *duplicate* code (as much as realistically possible)
- All reusable non-react code and associated types moved to a single file "lib.ts"
- All reusable react code and associated component-specific types moved to single file "libReact.tsx"

- Internationalization is incomplete (see `validateFormBuilderForm`) therefore it should either be taken seriously ASAP or removed
- Find out how to have a single error / loading / success type instead of hundreds of them (however success must be unique)
- Move all high-level constants and types to top of file, constants in all caps with underscores
- Use a standard font cascade that makes use of native fonts
- Reduce client side logging and monitoring if not being used
- Remove dynamic webpack imports - bundle size will be minimal after refactor (under ~3mb vs current 15mb)
- Move away from FrontAwesome (if it is paid and requires special npm source) and use something like feather icons or similar

# Infrastructure and Configuration (roughly in order)
- Use a lightweight NodeJS Docker container for building and developing locally
- Move all configuration URLs (currently embedded in source code) to .env files and use the `dotenv` package to load them. .env.prod and .env.dev may be used for their respective intents.
- Minimize package.json script usage and prefer docker + shell
- Use nginx or similar proxy for building and running in production (faster, more secure than express)
- Merge `platform-client` into the `mono-platform` repo so Front End and Back End code co-exist

# Programming Standards
- Avoid regex and string-type programming
- Only store configuration externally (.env files)
- Avoid creating new functions and components unless they are being reused. "Breaking things down" does not help, unless it is for reuse.
- Avoid null and undefined

# Timeline and distribution of work
- 3 weeks of development freeze in December
- Work is distributed by technical domain rather than feature domain - improvements are made wholesale across code base rather than per-feature area. Risk is mitigated my code review (reviewer must have a strong understanding of what may cause a breaking change.)

# Idea, Questions
- Create a global solution for error/success/loading states unlike the current redux actions that have those states duplicated. (research the use of https://github.com/tc39/proposal-error-cause paired with React error boundaries for catching errors at a high level while retaining error info)
- Centralize and deduplicate networking code

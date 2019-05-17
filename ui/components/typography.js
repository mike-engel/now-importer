import styled from "styled-components";
import { propOr } from "ramda";
import { black } from "./color";
import { spacing } from "../utils/spacing";

// enum
export const FontStyle = {
  Italic: "italic",
  Normal: "normal",
  Inherit: "inherit"
};

// enum
export const FontWeight = {
  Regular: "400",
  Semibold: "500",
  Inherit: "inherit"
};

export const fontFamily =
  "system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif";

// Perfect fourth
export const fontSize = {
  level1: "2.0000rem",
  level2: "1.4375rem",
  level3: "1.0000rem",
  level4: "0.7500rem",
  level5: "10px",
  inherit: "1em"
};

export const Heading = styled("div").attrs(({ level }) => ({
  role: "heading",
  "aria-level": level || 1
}))`
  max-width: 40em;
  color: ${propOr(black, "color")};
  font-size: ${({ level, displayLevel }) => fontSize[`level${displayLevel || level || 1}`]};
  font-weight: ${propOr(FontWeight.Semibold, "fontWeight")};
  font-style: ${propOr(FontStyle.Normal, "fontStyle")};
  line-height: 1.2;
  margin: 0;

  & + * {
    margin-top: ${spacing(2)}px !important;
  }
`;

export const Text = styled("p")`
  max-width: 40em;
  color: ${propOr(black, "color")};
  font-size: ${({ level, displayLevel }) => fontSize[`level${displayLevel || level || 3}`]};
  font-weight: ${propOr(FontWeight.Regular, "fontWeight")};
  font-style: ${propOr(FontStyle.Normal, "fontStyle")};
  line-height: 1.4;
  margin: 0;

  & + * {
    margin-top: ${spacing(2)}px !important;
  }
`;

export const Span = styled(Text).attrs(() => ({ as: "span" }))``;

Span.defaultProps = {
  color: "inherit",
  fontWeight: FontWeight.Inherit,
  fontStyle: FontStyle.Inherit
};

export const Link = styled("a")`
  display: inline-block;
  text-decoration: none;
  color: ${propOr(black, "color")};
  font-size: ${({ level, displayLevel }) => fontSize[`level${displayLevel || level || 3}`]};
  font-weight: ${propOr(FontWeight.Regular, "fontWeight")};
  font-style: ${propOr(FontStyle.Normal, "fontStyle")};
  border-bottom: 1px solid ${black};
  line-height: 1.2;
  margin: 0;
  transition: color 250ms, border-color 250ms;

  @media (hover) {
    &:hover {
      color: ${black};
      border-color: ${black};
    }
  }
`;

Link.defaultProps = {
  fontWeight: FontWeight.Inherit,
  fontStyle: FontStyle.Inherit,
  level: "inherit"
};

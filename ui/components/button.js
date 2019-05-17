import styled from "styled-components";
import { spacing } from "../utils/spacing";
import { fontFamily, FontWeight } from "./typography";
import { black, white } from "./color";

export const Button = styled("button")`
  padding: ${spacing()}px ${spacing(2)}px;
  margin: 0;
  display: block;
  width: 100%;
  appearance: none;
  font-family: ${fontFamily};
  font-size: 1rem;
  font-weight: ${FontWeight.Semibold};
  border: 1px solid ${black};
  border-radius: ${spacing(0.5)}px;
  color: ${white};
  background: ${black};
  opacity: ${({ disabled }) => (disabled ? "0.25" : "1.0")};
  pointer-events: ${({ disabled }) => (disabled ? "none" : "initial")};
  transition: background 150ms, border-color 150ms, color 150ms;
  text-align: center;
  text-decoration: none;

  @media (hover) {
    &:hover {
      cursor: pointer;
      background: rgba(0, 0, 0, 0.9);
      border-color: rgba(0, 0, 0, 0.9);
    }
  }
`;

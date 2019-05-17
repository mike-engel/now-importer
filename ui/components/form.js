import styled from "styled-components";
import { spacing } from "../utils/spacing";
import { black, white, darkGrey } from "./color";
import { Span } from "./typography";

const RawInput = ({
  className,
  id,
  label: { value: labelValue, ...restLabel },
  placeholder,
  disabled,
  type = "text",
  name,
  value,
  onChange
}) => (
  <label {...restLabel} className={className} htmlFor={id}>
    <Span>{labelValue}</Span>
    <input
      type={type}
      name={name}
      id={id}
      placeholder={placeholder}
      value={value}
      onChange={evt => onChange(evt.currentTarget.value)}
      disabled={disabled}
    />
  </label>
);

export const Input = styled(RawInput)`
  position: relative;
  padding: 0;
  margin: 0 0 ${spacing(2)}px;
  display: block;

  input {
    display: block;
    width: 100%;
    font-family: inherit;
    font-size: 1rem;
    color: ${black};
    padding: ${spacing(1)}px ${spacing(1)}px;
    border-radius: ${spacing(0.25)}px;
    background: ${white};
    border: 1px solid ${darkGrey};
    transition: border 250ms;
    opacity: ${({ disabled }) => (disabled ? "0.5" : "1.0")};
    margin-top: 0 !important;

    &:focus {
      border-color: ${black};
    }
  }

  ${Span} {
    display: inline-block;
    background: ${white};
    padding: 0;
    margin: 0 0 ${spacing(0.5)}px 0;
    transition: color 250ms;
    color: ${darkGrey};
  }
`;

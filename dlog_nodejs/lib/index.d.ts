export interface Options {
    sanitize_emails?: boolean | undefined;
    sanitize_credit_cards?: boolean | undefined;
}

export function configure (api_key: string): undefined;

export function with_dlog<T>(api_key: string, handler: T): T;
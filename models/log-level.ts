/* tslint:disable */
/* eslint-disable */
/**
 * NOTE: This class is auto generated by OpenAPI Generator (https://openapi-generator.tech).
 * Do not edit the class manually.
 *
 * Jellyfin API
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */



/**
 * 
 * @export
 * @enum {string}
 */

export const LogLevel = {
    Trace: 'Trace',
    Debug: 'Debug',
    Information: 'Information',
    Warning: 'Warning',
    Error: 'Error',
    Critical: 'Critical',
    None: 'None'
} as const;

export type LogLevel = typeof LogLevel[keyof typeof LogLevel];




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


import { PersonLookupInfo } from './person-lookup-info';

/**
 * 
 * @export
 * @interface PersonLookupInfoRemoteSearchQuery
 */
export interface PersonLookupInfoRemoteSearchQuery {
    /**
     * 
     * @type {PersonLookupInfo}
     * @memberof PersonLookupInfoRemoteSearchQuery
     */
    'SearchInfo'?: PersonLookupInfo;
    /**
     * 
     * @type {string}
     * @memberof PersonLookupInfoRemoteSearchQuery
     */
    'ItemId'?: string;
    /**
     * Gets or sets the provider name to search within if set.
     * @type {string}
     * @memberof PersonLookupInfoRemoteSearchQuery
     */
    'SearchProviderName'?: string | null;
    /**
     * Gets or sets a value indicating whether disabled providers should be included.
     * @type {boolean}
     * @memberof PersonLookupInfoRemoteSearchQuery
     */
    'IncludeDisabledProviders'?: boolean;
}

